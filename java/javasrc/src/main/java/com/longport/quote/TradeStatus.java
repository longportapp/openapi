package com.longport.quote;

/**
 * Security trading status
 */
public enum TradeStatus {
    /** Normal */
    Normal,
    /** Suspension */
    Halted,
    /** Delisted */
    Delisted,
    /** Fuse */
    Fuse,
    /** Prepare List */
    PrepareList,
    /** Code Moved */
    CodeMoved,
    /** To Be Opened */
    ToBeOpened,
    /** Split Stock Halts */
    SplitStockHalts,
    /** Expired */
    Expired,
    /** Warrant To BeListed */
    WarrantPrepareList,
    /** Suspend */
    SuspendTrade,
}
