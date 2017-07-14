
interface i1 {
  val a: Int
}

interface i2 {
  val b: Int
}

interface i3 : i1, i2 {
  val c: Int
}

class C1 (override val a: Int, override val b: Int, override val c: Int) : i3

fun main (args: Array<String>) {
  val c1 = C1 (1, 2, 3)
  println ("${c1.a} ${c1.b} ${c1.c}")
}
