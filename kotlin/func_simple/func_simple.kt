
fun a (a_ : Int, b_ : Int) : Int = a_ + b_

fun b (a_ : Int, b_ : Int) = a_ + b_

fun c (a_ : Int, b_ : Int) : Int {
  return a_ + b_
}

fun d (a : (b : Int, c : Int) -> Int, b: Int, c: Int) = a (b, c)

fun e (a : (b : Int, c : Int) -> Int, b: Int, c: Int) : Int {
  return a (b, c)
}

fun main (args: Array<String>) {
//  val _a = a (1, 2)
//  print (a (1,2) as String + "\n")
//  print (_a as? String + "\n")

  /*
   * There really must be a better way of doing this, why the 
   * fuck can't you just have the func call rtn be cast to
   * an integer explicitly or implicity... makes no sense
   */
  print ("${a (1, 2)}" + "\n")
  print ("${b (1, 3)}" + "\n")
  print ("${c (1, 4)}" + "\n")
  print ("${d (::a, 1, 5)}" + "\n")
  print ("${e (::a, 1, 6)}" + "\n")
  print ("${d ({ x, y -> x+y}, 1, 7)}" + "\n")
  print ("${e ({ x, y -> x+y}, 1, 8)}" + "\n")

}

