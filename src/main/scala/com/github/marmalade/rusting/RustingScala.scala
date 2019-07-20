package com.github.marmalade.rusting

import java.nio.file.Paths

object RustingScala {

  {
    val p = Paths.get("rusting/target/release/" + System.mapLibraryName("rusting"))
    System.load(p.toAbsolutePath.toString)
  }

  @native def fact(i: Int): Int

  @native def greetings(name: String): String

  def main(args: Array[String]): Unit = {
    println(s"Factorial of 7 is ${fact(7)}")
    println(greetings("Joe"))
  }

}

