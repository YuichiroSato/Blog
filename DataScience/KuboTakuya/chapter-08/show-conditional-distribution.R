as <- seq(0, 3)
bs <- seq(0, 2)
la <- length(as)
lb <- length(bs)

jointDistribution <- function() {
  vec <- vector(length = la * lb)
  i <- 1
  for (a in as) {
    pa <- dbinom(a, la - 1, 0.5)
    for (b in bs) {
      pb <- dbinom(b, lb - 1, 0.1 * a + 0.1)
      vec[i] <- pa * pb
      i <- i + 1
    }
  }
  t(matrix(vec, nrow = lb, ncol = la))
}

conditionalGivenA <- function(joint, a) {
  joint[a + 1,] / sum(joint[a + 1,])
}

conditionalGivenB <- function(joint, b) {
  joint[,b + 1] / sum(joint[,b + 1])
}

showJoint <- function(joint) {
  cat("Joint distribution, p(a,b)\n")
  cat("a\\b |", paste0(bs, "\t\t"), "\n")
  for (a in as) {
    cat(a, "  |", paste0(joint[a + 1,], "\t"), "\n")
  }
  cat("\n")
}

showConditional <- function(is, joint, f) {
  f <- match.fun(f)
  for (i in is) {
    cat(i, "  |", paste0(f(joint, i), "\t"), "\n")
  }
  cat("\n")
}

joint <- jointDistribution()
showJoint(joint)

cat("Conditional distribution, p(b|a=i)\n")
cat("a\\b |", paste0(bs, "\t\t"), "\n")
showConditional(as, joint, conditionalGivenA)

cat("Conditional distribution, p(a|b=i)\n")
cat("b\\a |", paste0(as, "\t\t\t"), "\n")
showConditional(bs, joint, conditionalGivenB)
