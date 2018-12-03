import kotlin.test.assertEquals
import org.junit.Test

class TestSource {
    @Test fun partOne() {
        var input = listOf("#1 @ 1,3: 4x4",
                                        "#2 @ 3,1: 4x4",
                                        "#3 @ 5,5: 2x2")
        var sol = computeSolutionPartOne(input)
        assertEquals(sol, 4)
    }

    @Test fun partTwo() {
        var input = listOf("#1 @ 1,3: 4x4",
                                        "#2 @ 3,1: 4x4",
                                        "#3 @ 5,5: 2x2")
        var sol = computeSolutionPartTwo(input)
        assertEquals(sol, 3)
    }
}
