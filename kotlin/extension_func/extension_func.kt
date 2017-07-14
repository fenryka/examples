
class bob (var _a : Int) {
  var _b = 10

  fun add_b (a_ : Int) : Int {
    val tmp = _b
    _b += a_
    return tmp
  }
}

// extend bob with a new method
fun bob.add_a (a_: Int) {
  _a += a_
}

fun main (args: Array<String>) {
  var b = bob (100)

  println ("${b._a} ${b._b}")
  println ("${b.add_b (50)}")
  println ("${b._b}")
  println ("${b.add_b (40)}")
  println ("${b._a} ${b._b}")

  b.add_a (100)

  println ("${b._a} ${b._b}")
}
