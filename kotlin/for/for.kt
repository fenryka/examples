
fun check (v: Int) : Boolean {
  if (v != 1)
    throw IllegalArgumentException("Name required")
  return true
}

fun test (l_: List<Int>) {
  println (l_)

  val found = l_.find { try { check(it); true } catch (e: IllegalArgumentException) { false } } != null

  if (found) {
    println ("YES")
  }
  else {
    println ("NO")
  }
}

fun main (args: Array<String>) {
  var l1 = listOf (1, 0, 0, 0, 0)
  var l2 = listOf (0, 0, 0, 1, 0)
  var l3 = listOf (0, 0, 0, 0, 0)

  test(l1)
  test(l2)
  test(l3)
}
