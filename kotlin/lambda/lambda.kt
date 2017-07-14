
fun wrapper1 (a : () -> Int) = a()

class bob (var _a : Int) {
  fun operate (a_ : (Int) -> Int) {
    _a = a_(_a)
  }
}

object plop {
  fun operate (a_: () -> Any) {
    a_()
  }
}

fun main (args: Array<String>) {
  println ("${wrapper1 { 3 }}")

  var b = bob (1)

  b.operate ({ x : Int -> x + 1 }); println (b._a)
  b.operate ({ x -> x + 8 });       println (b._a)
  b.operate { x -> x / 2 };         println (b._a)
  b.operate { it - 3 };             println (b._a)

  plop.operate { println (b._a) }
}
