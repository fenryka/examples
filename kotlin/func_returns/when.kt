
fun test (a_ : Int, b_ : String) = {
  var a = a_ + 1
  print (a)
  b_
}

fun test2 (a_ : Int, b_ : String) : String {
  var a = a_ + 1
  print (a)
  return b_
}

fun test3 (a_ : Int, b_ : String) : Int {
  val a : Int = run {
    print (a_)
    print (b_)
    a_ + 1
  }
  return a
}

fun test4 (a_ : Int, b_ : String) : Int {
  val a : Int = {
    print (a_)
    print (b_)
    a_ + 1
  }()

  return a
}

fun main (args: Array<String>) {
  println (test  (1, "test1"))
  println (test2 (2, "test2"))
  println (test3 (3, "test3"))
  println (test4 (4, "test4"))
}
