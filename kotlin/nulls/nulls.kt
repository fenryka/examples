
/*
 * Int? just means a can be null
 */
fun test (a : Int?) {
  println (a)

  a?.let {
    println (a+1)
  }
}

fun main (args: Array<String>) {
  /* Int? allows a to be null */
  var a : Int? = null

  test (a)

  a = 1

  test (a)
}
