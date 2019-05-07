use futures::sync::mpsc::{channel, Receiver, Sender};

use graph::prelude::*;

pub struct MockBlockStream {
    chain_head_update_sink: Sender<ChainHeadUpdate>,
    _chain_head_update_stream: Receiver<ChainHeadUpdate>,
}

impl MockBlockStream {
    fn new() -> Self {
        let (chain_head_update_sink, chain_head_update_stream) = channel(100);

        Self {
            chain_head_update_sink,
            _chain_head_update_stream: chain_head_update_stream,
        }
    }
}

impl Stream for MockBlockStream {
    type Item = EthereumBlockWithTriggers;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<EthereumBlockWithTriggers>>, Error> {
        Ok(Async::Ready(None))
    }
}

impl EventConsumer<ChainHeadUpdate> for MockBlockStream {
    fn event_sink(&self) -> Box<Sink<SinkItem = ChainHeadUpdate, SinkError = ()> + Send> {
        Box::new(self.chain_head_update_sink.clone().sink_map_err(|_| ()))
    }
}

impl BlockStream for MockBlockStream {
    fn parse_triggers(
        _log_filter_opt: Option<EthereumLogFilter>,
        _call_filter_opt: Option<EthereumCallFilter>,
        _block_filter_opt: Option<EthereumBlockFilter>,
        _include_calls_in_blocks: bool,
        _descendant_block: EthereumBlockWithCalls,
    ) -> Result<EthereumBlockWithTriggers, Error> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct MockBlockStreamBuilder;

impl MockBlockStreamBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl BlockStreamBuilder for MockBlockStreamBuilder {
    type Stream = MockBlockStream;

    fn build(
        &self,
        _logger: Logger,
        _deployment_id: SubgraphDeploymentId,
        _log_filter: Option<EthereumLogFilter>,
        _call_filter: Option<EthereumCallFilter>,
        _block_filter: Option<EthereumBlockFilter>,
        _include_calls_in_blocks: bool,
    ) -> Self::Stream {
        MockBlockStream::new()
    }
}
