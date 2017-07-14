
fun main (args: Array<String>) {
  val a = Array<Int>(10, { 2 } )
  val b = 10

  a[0] = 1

  a.forEach { print ("$it ") }
  print ("\n")
  println (a::javaClass)
  println (a::class.java)
  println (b::class.java)
}
