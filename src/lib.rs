//! Graph-based route and cycle search for [`evm-amm-state`](evm_amm_state).
//!
//! This crate does not discover, cold-start, or synchronize pools. Those jobs
//! remain with `evm-amm-state`. `evm-amm-search` takes an already-populated
//! [`AdapterRegistry`](evm_amm_state::adapters::AdapterRegistry), builds a token
//! graph from its ready pool metadata, and evaluates candidate routes by calling
//! each pool adapter's canonical `simulate_swap`.

#[cfg(all(feature = "alloy1", feature = "alloy2"))]
compile_error!("features `alloy1` and `alloy2` are mutually exclusive; pass `--no-default-features --features alloy2`");

#[cfg(not(any(feature = "alloy1", feature = "alloy2")))]
compile_error!("exactly one of features `alloy1` or `alloy2` must be enabled");

pub use alloy_transport_balancer::{
    alloy_json_rpc, alloy_transport, alloy_transport_http, reqwest,
};
pub use evm_fork_cache::{
    alloy_consensus, alloy_eips, alloy_network, alloy_provider, alloy_pubsub, alloy_rpc_client,
    alloy_rpc_types_eth, revm,
};

#[cfg(feature = "alloy1")]
pub(crate) fn execution_gas_used<H>(result: &revm::context::result::ExecutionResult<H>) -> u64 {
    result.gas_used()
}

#[cfg(feature = "alloy2")]
pub(crate) fn execution_gas_used<H>(result: &revm::context::result::ExecutionResult<H>) -> u64 {
    result.tx_gas_used()
}

mod demo_router;
mod graph;
mod liquidity;
#[cfg(feature = "live-runtime")]
mod live_graph;
#[cfg(feature = "live-runtime")]
mod live_routes;
mod overlay_cache;
mod search;

pub use demo_router::{
    DEMO_ROUTER, DemoRouterConfig, DemoRouterHop, GenericExecution, SwapGasEstimate,
    demo_router_hops_for_quote, demo_router_runtime, encode_demo_router_execute_calldata,
    encode_demo_router_execute_from_calldata, install_demo_router, simulate_gas_hops,
    simulate_route_gas, simulate_route_prefix_gas,
};
#[cfg(feature = "live-runtime")]
pub use demo_router::{
    prewarm_demo_router_token_transfer, simulate_versioned_route_gas,
    simulate_versioned_route_gas_with_balance_mappings,
};
pub use graph::{
    AmmGraph, EdgeData, GraphBuildOptions, GraphBuildReport, GraphBuildSummary, GraphPoolMutation,
    GraphVersion, SkippedPool, SkippedPoolReason, StableAmmDiGraph,
};
pub use liquidity::{
    BalanceState, LiquidityIndexScope, LiquidityPruneStats, LiquidityPruningConfig,
    PoolLiquidityBuildReport, PoolLiquidityError, PoolLiquidityIndex, PoolLiquidityMutationReport,
    PoolLiquidityRefreshFailure, PoolLiquidityRefreshReport, PoolLiquidityTracker,
    TransferApplyReport, TransferEventSource,
};
#[cfg(feature = "live-runtime")]
pub use live_graph::{
    GraphDelta, GraphPoolDelta, GraphTopologyImpact, IndexedPool, LiveAmmGraph, LiveGraphError,
};
#[cfg(feature = "live-runtime")]
pub use live_routes::{
    LiveRouteObserver, LiveRouteObserverError, LiveRouteRuntime, LiveRouteRuntimeConfig,
    LiveRouteRuntimeError, LiveRouteRuntimeEvent, LiveRouteRuntimeEventKind,
    LiveRouteRuntimeFailure, LiveRouteRuntimeHandle, LiveRouteSubscription, RouteCancellationToken,
    RouteInvalidationReason, RouteJobStamp, RouteProvenance, RouteSearchFailure, RouteSearchJobId,
    RouteSearchTrigger, RouteSubscriptionId, RouteSubscriptionSnapshot, RouteSubscriptionSpec,
    RouteSubscriptionState, VersionedRouteQuote,
};
#[cfg(feature = "live-runtime")]
pub use search::LiveSearchView;
pub use search::{
    AdaptiveEdgeShortlistConfig, AffectedPools, AmmSearcher, BatchSearchReport, CycleQuote,
    CycleRequest, FastLaneConfig, HeuristicSearchConfig, Hop, HopQuote,
    IncrementalRouteUpdateReport, IncrementalRouteUpdateStatus, ParallelSearchConfig, PathFailure,
    QuoteCacheStats, RecomputeReason, RoutePath, RouteQuote, RouteRequest, RouteSearchEvent,
    RouteSearchPhase, RouteSearchProgress, RouteSearchSession, RouteUpdateEvent, SearchConfig,
    SearchControl, SearchError, SearchFinality, SearchMode, StreamedRouteStatus,
    StreamingCompletion, StreamingPhaseStats, StreamingSearchConfig, StreamingSearchReport,
    StreamingThresholdMode, StreamingThresholdPolicy, UpperBoundPruningConfig,
};
