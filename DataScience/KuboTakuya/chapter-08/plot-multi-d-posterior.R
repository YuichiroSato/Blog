ys <- seq(0, 20, by = 5)
qs <- seq(0, 5)

likelihood <- function(ys, q) {
  prod(dpois(ys, lambda = q))
}

prior <- function(q) {
  dbinom(q, 5, 0.2)
}

marginalOfY <- function(ys) {
  joint <- function(q) {
    likelihood(ys, q) * prior(q)
  }
  s <- 0.0
  for (q in qs) {
    s <- s + joint(q)
  }
  s
}

posteriorGivenY <- function(ys) {
  function(qs) {
    a <- vector(length = length(qs))
    for (i in seq(1, length(qs))) {
      a[i] = likelihood(ys, qs[i]) * prior(qs[i]) / marginalOfY(ys)
    }
    a
  }
}

xl <- "q"
yl <- "Probability"
for (i in c(1, 5, 10)) {
  for (j in c(1, 5, 10)) {
    for (k in c(1, 5, 10)) {
      ys <- c(i, j, k)
      title <- paste0("Posterior distribution given Y, Y = {", i, ", ", j, ", ", k, "}")
      plot(0, 0, type = "n", xlim = c(0, 5), ylim = c(0.0, 1.0), main = title, xlab = xl, ylab = yl)
      d <- posteriorGivenY(ys)
      lines(qs, d(qs), type = "l")
      points(qs, d(qs))
    }
  }
}
