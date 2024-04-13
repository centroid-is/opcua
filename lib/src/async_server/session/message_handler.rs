use std::sync::Arc;

use parking_lot::RwLock;
use tokio::task::JoinHandle;

use crate::{
    async_server::{
        info::ServerInfo,
        node_manager::{BrowseNode, NodeManager, ReadNode},
    },
    server::prelude::{
        BrowseNextRequest, BrowseNextResponse, BrowseRequest, BrowseResponse, BrowseResult,
        ByteString, ReadRequest, ReadResponse, ResponseHeader, ServiceFault, StatusCode,
        SupportedMessage, TimestampsToReturn,
    },
};

use super::{controller::Response, instance::Session};

type NodeManagers = Vec<Arc<dyn NodeManager + Send + Sync + 'static>>;

pub(crate) struct MessageHandler {
    node_managers: NodeManagers,
    info: Arc<ServerInfo>,
}

pub(crate) enum HandleMessageResult {
    AsyncMessage(JoinHandle<Response>),
    SyncMessage(Response),
}

struct Request<T> {
    pub request: Box<T>,
    pub request_id: u32,
    pub request_handle: u32,
    pub info: Arc<ServerInfo>,
    pub session: Arc<RwLock<Session>>,
}

impl<T> Request<T> {
    pub fn new(
        request: Box<T>,
        info: Arc<ServerInfo>,
        request_id: u32,
        request_handle: u32,
        session: Arc<RwLock<Session>>,
    ) -> Self {
        Self {
            request,
            request_id,
            request_handle,
            info,
            session,
        }
    }

    pub fn response(&self, message: impl Into<SupportedMessage>) -> Response {
        Response {
            message: message.into(),
            request_handle: self.request_handle,
            request_id: self.request_id,
        }
    }

    pub fn service_fault(&self, status_code: StatusCode) -> Response {
        Response {
            message: ServiceFault::new(self.request_handle, status_code).into(),
            request_handle: self.request_handle,
            request_id: self.request_id,
        }
    }
}

impl MessageHandler {
    pub fn new(info: Arc<ServerInfo>) -> Self {
        Self {
            node_managers: Default::default(),
            info,
        }
    }

    pub fn handle_message(
        &mut self,
        message: SupportedMessage,
        session: Arc<RwLock<Session>>,
        request_id: u32,
    ) -> HandleMessageResult {
        let request_handle = message.request_handle();
        // Session management requests are not handled here.
        match message {
            SupportedMessage::ReadRequest(request) => {
                HandleMessageResult::AsyncMessage(tokio::task::spawn(Self::read(
                    self.node_managers.clone(),
                    Request::new(
                        request,
                        self.info.clone(),
                        request_id,
                        request_handle,
                        session,
                    ),
                )))
            }

            SupportedMessage::BrowseRequest(request) => {
                HandleMessageResult::AsyncMessage(tokio::task::spawn(Self::browse(
                    self.node_managers.clone(),
                    Request::new(
                        request,
                        self.info.clone(),
                        request_id,
                        request_handle,
                        session,
                    ),
                )))
            }

            SupportedMessage::BrowseNextRequest(request) => {
                HandleMessageResult::AsyncMessage(tokio::task::spawn(Self::browse_next(
                    self.node_managers.clone(),
                    Request::new(
                        request,
                        self.info.clone(),
                        request_id,
                        request_handle,
                        session,
                    ),
                )))
            }

            message => {
                debug!(
                    "Message handler does not handle this kind of message {:?}",
                    message
                );
                HandleMessageResult::SyncMessage(Response {
                    message: ServiceFault::new(
                        message.request_header(),
                        StatusCode::BadServiceUnsupported,
                    )
                    .into(),
                    request_id,
                    request_handle,
                })
            }
        }
    }

    async fn read(node_managers: NodeManagers, request: Request<ReadRequest>) -> Response {
        // TODO: Figure out how to handle permissions in a good way.
        // Tie it to general access control, so that each session is assigned an opaque token
        // by a central access manager, then we check permission on _that_ instead of relying on the
        // session. In the end we will probably pass a kind of generic "request context"
        // which contains methods for calling an external auth manager.
        let num_nodes = request
            .request
            .nodes_to_read
            .as_ref()
            .map(|r| r.len())
            .unwrap_or_default();
        if num_nodes == 0 {
            return request.service_fault(StatusCode::BadNothingToDo);
        }
        if request.request.max_age < 0.0 {
            return request.service_fault(StatusCode::BadMaxAgeInvalid);
        }
        if request.request.timestamps_to_return == TimestampsToReturn::Invalid {
            return request.service_fault(StatusCode::BadTimestampsToReturnInvalid);
        }
        if num_nodes > request.info.operational_limits.max_nodes_per_read {
            return request.service_fault(StatusCode::BadTooManyOperations);
        }
        let mut results: Vec<_> = request
            .request
            .nodes_to_read
            .unwrap_or_default()
            .into_iter()
            .map(|n| ReadNode::new(n))
            .collect();

        for node_manager in node_managers {
            if let Err(e) = node_manager
                .read(
                    request.request.max_age,
                    request.request.timestamps_to_return,
                    &mut results,
                )
                .await
            {
                for node in &mut results {
                    if node_manager.owns_node(&node.node().node_id) {
                        node.set_error(e);
                    }
                }
            }
        }

        let results = results.into_iter().map(|r| r.take_result()).collect();

        Response {
            message: ReadResponse {
                response_header: ResponseHeader::new_good(request.request_handle),
                results: Some(results),
                diagnostic_infos: None,
            }
            .into(),
            request_id: request.request_id,
            request_handle: request.request_handle,
        }
    }

    async fn browse(node_managers: NodeManagers, request: Request<BrowseRequest>) -> Response {
        let num_nodes = request
            .request
            .nodes_to_browse
            .as_ref()
            .map(|r| r.len())
            .unwrap_or_default();
        if num_nodes == 0 {
            return request.service_fault(StatusCode::BadNothingToDo);
        }
        if !request.request.view.view_id.is_null() || !request.request.view.timestamp.is_null() {
            info!("Browse request ignored because view was specified (views not supported)");
            return request.service_fault(StatusCode::BadViewIdUnknown);
        }
        if num_nodes > request.info.operational_limits.max_nodes_per_browse {
            return request.service_fault(StatusCode::BadTooManyOperations);
        }

        let max_references_per_node = if request.request.requested_max_references_per_node == 0 {
            request
                .info
                .operational_limits
                .max_references_per_browse_node
        } else {
            request
                .info
                .operational_limits
                .max_references_per_browse_node
                .min(request.request.requested_max_references_per_node as usize)
        };

        let nodes: Vec<_> = request
            .request
            .nodes_to_browse
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(idx, r)| BrowseNode::new(r, max_references_per_node, idx))
            .collect();

        let mut results: Vec<_> = (0..nodes.len()).map(|_| None).collect();

        Self::execute_browse_inner(node_managers, nodes, &mut results, request.session).await;

        Response {
            message: BrowseResponse {
                response_header: ResponseHeader::new_good(request.request_handle),
                results: Some(
                    // No option can be None here.
                    results.into_iter().map(Option::unwrap).collect(),
                ),
                diagnostic_infos: None,
            }
            .into(),
            request_handle: request.request_handle,
            request_id: request.request_id,
        }
    }

    async fn browse_next(
        node_managers: NodeManagers,
        request: Request<BrowseNextRequest>,
    ) -> Response {
        let num_nodes = request
            .request
            .continuation_points
            .as_ref()
            .map(|r| r.len())
            .unwrap_or_default();
        if num_nodes == 0 {
            return request.service_fault(StatusCode::BadNothingToDo);
        }
        if num_nodes > request.info.operational_limits.max_nodes_per_browse {
            return request.service_fault(StatusCode::BadTooManyOperations);
        }
        let mut results: Vec<_> = (0..num_nodes).map(|_| None).collect();

        let nodes = {
            let mut session = trace_write_lock!(request.session);
            let mut nodes = Vec::with_capacity(num_nodes);
            for (idx, point) in request
                .request
                .continuation_points
                .unwrap_or_default()
                .into_iter()
                .enumerate()
            {
                let point = session.remove_browse_continuation_point(&point);
                if let Some(point) = point {
                    nodes.push(BrowseNode::from_continuation_point(point, idx));
                } else {
                    results[idx] = Some(BrowseResult {
                        status_code: StatusCode::BadContinuationPointInvalid,
                        continuation_point: ByteString::null(),
                        references: None,
                    });
                }
            }
            nodes
        };

        Self::execute_browse_inner(node_managers, nodes, &mut results, request.session).await;

        Response {
            message: BrowseNextResponse {
                response_header: ResponseHeader::new_good(request.request_handle),
                results: Some(
                    // No option can be None here.
                    results.into_iter().map(Option::unwrap).collect(),
                ),
                diagnostic_infos: None,
            }
            .into(),
            request_handle: request.request_handle,
            request_id: request.request_id,
        }
    }

    async fn execute_browse_inner(
        node_managers: NodeManagers,
        mut nodes: Vec<BrowseNode>,
        results: &mut Vec<Option<BrowseResult>>,
        session: Arc<RwLock<Session>>,
    ) {
        let node_manager_count = node_managers.len();

        for (node_manager_index, node_manager) in node_managers.into_iter().enumerate() {
            if let Err(e) = node_manager.browse(&mut nodes).await {
                for node in &mut nodes {
                    if node_manager.owns_node(&node.node_id()) {
                        node.set_status(e);
                    }
                }
            }
            // Iterate over the current nodes, removing unfinished ones, and storing
            // continuation points when relevant.
            // This does not preserve ordering, for efficiency, so node managers should
            // not rely on ordering at all.
            // We store the input index to make sure the results are correctly ordered.
            let mut i = 0;
            let mut session = session.write();
            while let Some(n) = nodes.get(i) {
                if n.is_completed() {
                    let (result, cp, input_index) = nodes
                        .swap_remove(i)
                        .into_result(node_manager_index, node_manager_count);
                    results[input_index] = Some(result);
                    if let Some(c) = cp {
                        session.add_browse_continuation_point(c);
                    }
                } else {
                    i += 1;
                }
            }

            if nodes.is_empty() {
                break;
            }
        }

        for node in nodes {
            let (result, _, input_index) =
                node.into_result(node_manager_count - 1, node_manager_count);
            results[input_index] = Some(result);
        }
    }
}
