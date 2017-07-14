

class me (val a: Int, val b: Int) {
}

inline fun <reified T> makeme (a: Int, b: Int) = makeme (T::class.java, a, b)

@Suppress ("UNCHECKED_CAST")
fun <T> makeme (c: Class<T>, a: Int, b: Int) : T {
  return when (c) {
    me::class.java -> me (a, b) as T
    Integer::class.java -> a as T
    else -> throw IllegalArgumentException ("pants")
  }
}

fun main (args: Array<String>) {
  var a = makeme<me> (1,2)

  println ("${a.a}, ${a.b}")

  var b: Int = makeme<Int> (2,3)
  println ("${b}")

  var c: Int = makeme (3,3)
  println ("${c}")
}
