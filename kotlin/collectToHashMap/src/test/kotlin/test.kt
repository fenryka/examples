
import org.junit.jupiter.api.Test;

class ToLHMTest {

    @Test
    fun test1() {
        val l = listOf(Pair("a", 1))
        val l1 = l.toLinkedHashMap()
        println (l1)
    }

    @Test
    fun test2() {
        val l = listOf(Pair("a", 1), Pair ("b", 1))
        val l1 = l.toLinkedHashMap()
        println (l1)
    }

    @Test
    fun test3() {
        val l = listOf(Pair("a", 1), Pair ("a", 32))
        val l1 = l.toLinkedHashMap()
        println (l1)
    }
}