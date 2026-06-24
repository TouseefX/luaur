//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_subtyping_should_cache_pairs_in_seen_set() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff_debug_luau_force_old_solver =
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let source = String::from(
        r#"
    type DataProxy = any

    type _Transaction = (c: _ApolloCache) -> ()
    type _ApolloCache = {
	    read: <T, TVariables>(self: _ApolloCache, query: Cache_ReadOptions<TVariables, T>) -> T | nil,
	    write: <TResult, TVariables>(self: _ApolloCache, write: Cache_WriteOptions<TResult, TVariables>) -> Reference | nil,
	    diff: <T>(self: _ApolloCache, query: Cache_DiffOptions) -> Cache_DiffResult<T>,
	    watch: (self: _ApolloCache, watch: Cache_WatchOptions<Record<string, any>>) -> (),
	    reset: (self: _ApolloCache) -> Promise<nil>,
	    evict: (self: _ApolloCache, options: Cache_EvictOptions) -> boolean,
	    restore: (self: _ApolloCache, serializedState: TSerialized_) -> _ApolloCache,
	    extract: (self: _ApolloCache, optimistic: boolean?) -> any,
	    removeOptimistic: (self: _ApolloCache, id: string) -> (),
	    batch: (self: _ApolloCache, options: Cache_BatchOptions<_ApolloCache>) -> (),
	    performTransaction: (self: _ApolloCache, transaction: _Transaction, optimisticId: string) -> (),
	    recordOptimisticTransaction: (self: _ApolloCache, transaction: _Transaction, optimisticId: string) -> (),
	    transformDocument: (self: _ApolloCache, document: DocumentNode) -> DocumentNode,
	    identify: (self: _ApolloCache, object: StoreObject | Reference) -> string | nil,
	    gc: (self: _ApolloCache) -> Array<string>,
	    modify: (self: _ApolloCache, options: Cache_ModifyOptions) -> boolean,
	    transformForLink: (self: _ApolloCache, document: DocumentNode) -> DocumentNode,
	    readQuery: <QueryType, TVariables>(
		    self: _ApolloCache,
		    options: Cache_ReadQueryOptions<QueryType, TVariables>,
		    optimistic: boolean?
	    ) -> QueryType | nil,
	    readFragment: <FragmentType, TVariables>(
		    self: _ApolloCache,
		    options: Cache_ReadFragmentOptions<FragmentType, TVariables>,
		    optimistic: boolean?
	    ) -> FragmentType | nil,
	    writeQuery: <TData, TVariables>(self: _ApolloCache, Cache_WriteQueryOptions<TData, TVariables>) -> Reference | nil,
	    writeFragment: <TData, TVariables>(
		    self: _ApolloCache,
		    Cache_WriteFragmentOptions<TData, TVariables>
	    ) -> Reference | nil,
    }

    export type ApolloCache<TSerialized> = {
	    -- something here needed
	    read: <T, TVariables>(self: ApolloCache<TSerialized>, query: Cache_ReadOptions<TVariables, T>) -> T | nil,
	    write: <TResult, TVariables>(
		    self: ApolloCache<TSerialized>,
		    write: Cache_WriteOptions<TResult, TVariables>
	    ) -> Reference | nil,
	    diff: <T>(self: ApolloCache<TSerialized>, query: Cache_DiffOptions) -> Cache_DiffResult<T>,
	    watch: (self: ApolloCache<TSerialized>, watch: Cache_WatchOptions<Record<string, any>>) -> (() -> ()),
	    reset: (self: ApolloCache<TSerialized>) -> Promise<nil>,
	    evict: (self: ApolloCache<TSerialized>, options: Cache_EvictOptions) -> boolean,
	    restore: (self: ApolloCache<TSerialized>, serializedState: TSerialized_) -> _ApolloCache,
	    extract: (self: ApolloCache<TSerialized>, optimistic: boolean?) -> TSerialized,
	    removeOptimistic: (self: ApolloCache<TSerialized>, id: string) -> (),
	    batch: (self: ApolloCache<TSerialized>, options: Cache_BatchOptions<_ApolloCache>) -> (),
	    performTransaction: (self: ApolloCache<TSerialized>, transaction: _Transaction, optimisticId: string) -> (),
	    -- bottom text
	    -- TOP
	    recordOptimisticTransaction: (
		    self: ApolloCache<TSerialized>,
		    transaction: _Transaction,
		    optimisticId: string
	    ) -> (),
	    transformDocument: (self: ApolloCache<TSerialized>, document: DocumentNode) -> DocumentNode,
	    identify: (self: ApolloCache<TSerialized>, object: StoreObject | Reference) -> string | nil,
	    gc: (self: ApolloCache<TSerialized>) -> Array<string>,
	    modify: (self: ApolloCache<TSerialized>, options: Cache_ModifyOptions) -> boolean,
	    -- BOTTOM

	    transformForLink: (self: ApolloCache<TSerialized>, document: DocumentNode) -> DocumentNode,
	    readQuery: <QueryType, TVariables>(
		    self: ApolloCache<TSerialized>,
		    options: Cache_ReadQueryOptions<QueryType, TVariables>,
		    optimistic: boolean?
	    ) -> QueryType | nil,
	    readFragment: <FragmentType, TVariables>(
		    self: ApolloCache<TSerialized>,
		    options: Cache_ReadFragmentOptions<FragmentType, TVariables>,
		    optimistic: boolean?
	    ) -> FragmentType | nil,
	    writeQuery: <TData, TVariables>(
		    self: ApolloCache<TSerialized>,
		    Cache_WriteQueryOptions<TData, TVariables>
	    ) -> Reference | nil,
	    writeFragment: <TData, TVariables>(
		    self: ApolloCache<TSerialized>,
		    Cache_WriteFragmentOptions<TData, TVariables>
	    ) -> Reference | nil,
    }


    export type InMemoryCache = ApolloCache<NormalizedCacheObject> & {
	    performTransaction: (
		    self: InMemoryCache,
		    update: (cache: InMemoryCache) -> ()
	    ) -> ()
    }

    type InMemoryCachePrivate = InMemoryCache & {
	    broadcastWatches: (self: InMemoryCachePrivate) -> (), -- ROBLOX NOTE: protected method
    }

    local InMemoryCache = {}
    InMemoryCache.__index = InMemoryCache

    -- InMemoryCache.batch = nil :: any
    function InMemoryCache:batch()
	    self = self :: InMemoryCachePrivate

	    if self.txCount == 0 then
		    self:broadcastWatches() --  problematic call?
	    end
    end
    "#,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let _ = fixture
        .base
        .check_string_optional_frontend_options(&source, None);
}
