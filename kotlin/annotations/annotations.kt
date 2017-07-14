import kotlin.reflect.KProperty

annotation class pants

@pants
class thing {
  var pants_i : Int

  init {
    pants_i = 10
  }
}

class thing2 {
  var pants_i : Int

  init {
    pants_i = 10
  }
}

fun main (args: Array<String>) {
  var t1 : thing  = thing()
  var t2 : thing2 = thing2()

  println (t1.pants_i)
  println (t2.pants_i)

  val p1 = t1.javaClass.kotlin.memberProperties.filter {
    it.findAnnotations<pants>() == null
  }

  println (p1)
}


