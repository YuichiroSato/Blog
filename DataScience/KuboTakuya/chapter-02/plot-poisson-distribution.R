x <- 0:20

for (i in 1:20) {
  Sys.setenv("DISPLAY"=":0")
  X11(type="cairo")
  title <- paste0("Poisson Distribution (lambda = ", i, ")")
  yl <- "Probability"
  plot(x, dpois(x, lambda = i), type = "b", lty = "solid", main = title, ylab = yl)
  savePlot(filename=paste0("pois", i, ".png"), type="png")
}
