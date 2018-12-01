import kotlin.test.assertEquals
import org.junit.Test

class TestSource {
    @Test fun partOne() {
        var input = "+1\n+1\n+1"
        var sol = computeSolutionPartOne(input)
        assertEquals(sol, 3)

        input = "+1\n+1\n-2"
        sol = computeSolutionPartOne(input)
        assertEquals(sol, 0)

        input = "-1\n-2\n-3"
        sol = computeSolutionPartOne(input)
        assertEquals(sol, -6)

        input = "+1\n-1"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 0)

        input = "+3\n+3\n+4\n-2\n-4"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 10)

        input = "-6\n+3\n+8\n+5\n-6"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 5)

        input = "+7\n+7\n-2\n-7\n-4"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 14)
    }

    @Test fun partTwo() {
        var input = "+1\n-1"
        var sol = computeSolutionPartTwo(input)
        assertEquals(sol, 0)

        input = "+3\n+3\n+4\n-2\n-4"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 10)

        input = "-6\n+3\n+8\n+5\n-6"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 5)

        input = "+7\n+7\n-2\n-7\n-4"
        sol = computeSolutionPartTwo(input)
        assertEquals(sol, 14)
    }
}
