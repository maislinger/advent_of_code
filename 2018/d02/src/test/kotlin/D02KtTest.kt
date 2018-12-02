import kotlin.test.assertEquals
import org.junit.Test

class TestSource {
    @Test fun partOne() {
        var input = listOf("abcdef",
                                        "bababc",
                                        "abbcde",
                                        "abcccd",
                                        "aabcdd",
                                        "abcdee",
                                        "ababab")
        var sol = computeSolutionPartOne(input)
        assertEquals(sol, 12)
    }

    @Test fun partTwo() {
        var input = listOf("abcde",
                                        "fghij",
                                        "klmno",
                                        "pqrst",
                                        "fguij",
                                        "axcye",
                                        "wvxyz")
        var sol = computeSolutionPartTwo(input)
        assertEquals(sol, "fgij")
    }
}
