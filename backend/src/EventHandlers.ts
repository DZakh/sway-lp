/*
 * Please refer to https://docs.envio.dev for a thorough guide on all Envio indexer features
 */
import { PoolContract, Pool, handlerContext, eventLog } from "generated";

const getPool = async (
  context: handlerContext,
  address: string,
  eventTime: number
): Promise<Pool> => {
  const pool = await context.Pool.get(address);
  if (!pool) {
    return {
      id: address,
      balance: 0n,
      createdAt: new Date(eventTime * 1000),
    };
  }
  return pool;
};

const getEventIdx = (event: eventLog<unknown>) => {
  return event.transaction.id + ":" + event.logIndex;
};

PoolContract.DepositEvent.handler(async ({ event, context }) => {
  const pool = await getPool(context, event.srcAddress, event.block.time);
  const newBalance = pool.balance + event.params.amount;
  context.Pool.set({
    ...pool,
    balance: newBalance,
  });
  context.PoolHistory.set({
    id: getEventIdx(event),
    pool_id: pool.id,
    balance: newBalance,
    time: new Date(event.block.time * 1000),
  });
});

PoolContract.Transfer.handler(async ({ event, context }) => {
  if (
    event.params.assetId ===
    "0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07"
  ) {
    const pool = await getPool(context, event.srcAddress, event.block.time);
    const newBalance = pool.balance - event.params.amount;
    context.Pool.set({
      ...pool,
      balance: newBalance,
    });
    context.PoolHistory.set({
      id: getEventIdx(event),
      pool_id: pool.id,
      balance: newBalance,
      time: new Date(event.block.time * 1000),
    });
  }
});
