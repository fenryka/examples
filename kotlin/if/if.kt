
fun test (a: Int) = if (a % 2 == 0) true else false

fun main (args: Array<String>) {
  for (i in 0..9) {
    var bob = if (test (i)) {
      print ("even ")
      "pants"
    } else {
      print ("odd  ")
      "fire"
    }
    println (bob)
  }

}
