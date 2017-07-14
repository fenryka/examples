
data class bob (val a_: Int = 10)

fun bobme (_a: Int = 2, _b: Int = 4) = with (bob(_b)) {
  a_ + _a
}

fun main (args: Array<String>) {

  println ("${bobme()}")
  println ("${bobme(10)}")
  println ("${bobme(_b=20)}")
  println ("${bobme(10, 20)}")
}
