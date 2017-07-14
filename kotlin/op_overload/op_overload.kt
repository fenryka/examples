
data class bob (var _a : Int)
  : Comparable<bob>
{
  /*
   * Not sure why this has to be declared as part of the class
   * and not done as a compantion function like plus 
   */
  override fun toString() = "A = ${_a}"

  override fun equals (other: Any?) : Boolean {
    if (other == null) return false
    if (other === this) return true

    return _a == (other as bob)._a
  }

  /*
   * override required else...
   *    'compareTo' hides member of supertype 'Comparable' and
   *    needs 'override' modifier
   */
  override fun compareTo (other: bob) : Int {
    return _a - other._a
  }
}

operator fun bob.plus (other: Int) = bob (_a + other)

operator fun bob.inc() = bob (_a + 1)

operator fun bob.contains (a_: Int) = a_ == _a

operator fun ClosedRange<bob>.iterator() : Iterator<bob> =
  object : Iterator<bob> {
    var curr = start

    override fun hasNext() = curr <= endInclusive
    /*
     * So these are functionally eqivalent
     * override fun next() = curr.apply { curr = bob(_a + 1) }
     * override fun next() : bob {
     *   val tmp = curr
     *   curr += 1
     *   return tmp
     * }
     */
    override fun next() = curr.apply { curr += 1 }
  }

fun main (args: Array<String>) {
  var b = bob (10)
  b += 1
  println (b)
  println (b++)
  println (++b)

  val b2 = bob(13)
  val b3 = bob(15)

  println (b == b2)
  println (b == b3)

  var b4 : bob? ; b4 = null

  println (b == b4)
  println (b == b)

  var b5 : bob? = bob (42)

  println (b5 == null)
  println (b5 != null)
  println (b5 == b4)

  println ("b < b5: ${b < (b5 as bob)}")

  val b6 = bob (10);

  println ("10 in b6: ${10 in b6}")
  println ("11 in b6: ${11 in b6}")

  println (bob(1)..bob(10))
  println ("bob(6) in bob(5)..bob(7) = ${(bob(6) in bob(5)..bob(7))}")
  println ("bob(8) in bob(5)..bob(7) = ${(bob(8) in bob(5)..bob(7))}")

  for (x in (bob(1)..bob(5))) {
    println ("  ${x}")
  }

}
