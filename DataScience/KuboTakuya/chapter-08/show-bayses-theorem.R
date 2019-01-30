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

marginalOfA <- function(joint, a) {
  apply(joint, 1, sum)[a + 1]
}

marginalOfB <- function(joint, b) {
  apply(joint, 2, sum)[b + 1]
}

showJoint <- function(joint) {
  cat("Joint distribution, p(a,b)\n")
  cat("a\\b |", paste0(bs, "\t\t"), "\n")
  for (a in as) {
    cat(a, "  |", paste0(joint[a + 1,], "\t"), "\n")
  }
  cat("\n")
}

showBayses <- function(f) {
  f <- match.fun(f)
  cat("a\\b |", paste0(bs, "\t\t"), "\n")
  for (a in as) {
    vec <- vector(length=lb)
    for (b in bs) {
      vec[b + 1] <- f(a, b)
    }
    cat(a, "  |", paste0(vec, "\t"), "\n")
  }
  cat("\n")
}

joint <- jointDistribution()
showJoint(joint)

cat("Joint distribution, p(a|b=i) * p(b=i)\n")
showBayses(function(a, b) conditionalGivenB(joint, b)[a + 1] * marginalOfB(joint, b))

cat("Joint distribution, p(b|a=i) * p(a=i)\n")
showBayses(function(a, b) conditionalGivenA(joint, a)[b + 1] * marginalOfA(joint, a))
