import assert from "assert";
import { 
  TestHelpers,
  Pool_DepositEvent
} from "generated";
const { MockDb, Pool } = TestHelpers;

describe("Pool contract DepositEvent event tests", () => {
  // Create mock db
  const mockDb = MockDb.createMockDb();

  // Creating mock for Pool contract DepositEvent event
  const event = Pool.DepositEvent.mock({data: {} /* It mocks event fields with default values, so you only need to provide data */});

  it("Pool_DepositEvent is created correctly", async () => {
    // Processing the event
    const mockDbUpdated = await Pool.DepositEvent.processEvent({
      event,
      mockDb,
    });

    // Getting the actual entity from the mock database
    let actualPoolDepositEvent = mockDbUpdated.entities.Pool_DepositEvent.get(
      `${event.chainId}_${event.block.height}_${event.logIndex}`
    );

    // Creating the expected entity
    const expectedPoolDepositEvent: Pool_DepositEvent = {
      id: `${event.chainId}_${event.block.height}_${event.logIndex}`,
    };
    // Asserting that the entity in the mock database is the same as the expected entity
    assert.deepEqual(actualPoolDepositEvent, expectedPoolDepositEvent, "Actual PoolDepositEvent should be the same as the expectedPoolDepositEvent");
  });
});
