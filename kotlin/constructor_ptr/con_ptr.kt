
data class bob (var a_: Int)

/* return type can be ommited as inferable */
//fun construct (a_: Int, b_: (Int) -> bob) : bob = b_(a_)
fun construct (a_: Int, b_: (Int) -> bob) = b_(a_)

fun construct (a_: (Int) -> bob) = a_(100)

fun main (args: Array<String>) {
  val a = construct (2, ::bob)
  println (a.a_)
  val b = construct (::bob)
  println (b.a_)
}
