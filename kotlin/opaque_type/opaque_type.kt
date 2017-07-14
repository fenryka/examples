
fun bob (p : Class<out Any?>) {
  println (p)

}

fun main (args: Array<String>) {
  bob (Integer::class.java)
  bob (1)
}
