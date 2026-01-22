import math
import time
from capsule import task


@task(name="test_timeout", timeout="2s", compute="HIGH")
def test_timeout() -> str:
    """
    This task should TIMEOUT because it sleeps longer than the configured timeout.
    """
    time.sleep(10)

    print("Sleep completed (this should NOT print)")
    return "TIMEOUT TEST FAILED - Should have timed out!"


@task(name="test_compute_limit_low", compute="LOW", timeout="60s")
def test_compute_limit_low() -> str:
    """
    This task should EXCEED compute limits with intensive CPU operations.
    """
    result = 0.0

    for i in range(100_000_000):
        result += math.sqrt(i) * math.sin(i) * math.cos(i)

        if i % 1000 == 0:
            result = result % 1_000_000

    print("Computation completed (this should NOT print)")
    return f"COMPUTE TEST FAILED - Should have run out of fuel! Result: {result}"


@task(name="test_compute_limit_medium", compute="MEDIUM", timeout="60s")
def test_compute_limit_medium() -> str:
    """
    This task should EXCEED MEDIUM compute limits.
    """
    result = 0.0

    for i in range(10_000_000_000):
        result += math.sqrt(i) * math.sin(i) * math.cos(i) * math.tan(i)

        if i % 10000 == 0:
            result = result % 1_000_000

    print("Computation completed (this should NOT print)")
    return f"COMPUTE TEST FAILED - Should have run out of fuel! Result: {result}"


@task(name="test_ram_limit", ram="10MB", compute="HIGH", timeout="30s")
def test_ram_limit() -> str:
    """
    This task should EXCEED RAM limits by allocating large lists.
    """
    arrays = []

    try:
        for i in range(1000):
            large_array = [idx * 0.5 for idx in range(1_000_000)]
            arrays.append(large_array)

            if i % 10 == 0:
                print(f"📊 Allocated {i} arrays (~{i * 8}MB)")

        print("Memory allocation completed (this should NOT print)")
        return f"RAM TEST FAILED - Should have run out of memory! Arrays: {len(arrays)}"
    except Exception as error:
        print(f"Python error: {error}")
        raise error


@task(name="test_combined_limits", timeout="3s", compute="LOW", ram="5MB")
def test_combined_limits() -> str:
    """
    This task tests multiple limits simultaneously:
    - Very low timeout
    - Low compute
    - Low RAM
    """
    data = []
    compute_result = 0.0

    for i in range(100):
        data.append([j * 0.5 for j in range(100_000)])

        for j in range(1_000_000):
            compute_result += math.sqrt(j) * math.sin(j)

        time.sleep(0.1)

    print("All operations completed (this should NOT print)")
    return f"COMBINED TEST FAILED - Should have hit a limit! Data: {len(data)}, Compute: {compute_result}"


@task(name="test_async_timeout", timeout="5s", compute="MEDIUM")
def test_async_timeout() -> str:
    """
    Tests timeout with multiple sleep operations.
    """
    delays = [2.0, 2.0, 2.0, 2.0, 2.0]

    for i, delay in enumerate(delays):
        time.sleep(delay)

    print("All delays completed (this should NOT print)")
    return "ASYNC TIMEOUT TEST FAILED - Should have timed out!"


@task(name="test_immediate_fuel_exhaustion", compute="LOW", timeout="60s")
def test_immediate_fuel_exhaustion() -> str:
    """
    This task should run out of fuel almost immediately.
    """
    result = 0.0
    iterations = 1_000_000_000

    for i in range(iterations):
        result += (
            math.pow(math.sqrt(i), 2)
            * math.sin(i)
            * math.cos(i)
            * math.tan(i)
            * math.log(i + 1)
        )

    print("Completed (this should NOT print)")
    return f"FUEL TEST FAILED - Result: {result}"


@task(name="main", compute="HIGH", timeout="120s")
def main() -> None:
    """
    Main function to demonstrate all limit tests.

    Uncomment individual tests to run them one at a time.
    Each test is designed to FAIL with a specific limit error.
    """
    # Test 1: Timeout
    print("\n🧪 Running Test 1: Timeout Limit")
    test_timeout()

    # Test 2: Low Compute
    print("\n🧪 Running Test 2: LOW Compute Limit")
    test_compute_limit_low()

    # Test 3: Medium Compute
    print("\n🧪 Running Test 3: MEDIUM Compute Limit")
    test_compute_limit_medium()

    # Test 4: RAM Limit
    print("\n🧪 Running Test 4: RAM Limit")
    test_ram_limit()

    # Test 5: Combined Limits
    print("\n🧪 Running Test 5: Combined Limits")
    test_combined_limits()

    # Test 6: Async Timeout
    print("\n🧪 Running Test 6: Async Timeout")
    test_async_timeout()

    # Test 7: Immediate Fuel Exhaustion
    print("\n🧪 Running Test 7: Immediate Fuel Exhaustion")
    test_immediate_fuel_exhaustion()

    print("\n✅ Limit testing completed.")
