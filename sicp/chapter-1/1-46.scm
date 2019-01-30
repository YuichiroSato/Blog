(define (square x) (* x x))

(define (average x y) (/ (+ x y) 2.0))

(define (average-damp f)
  (lambda (x) (average x (f x))))

(define (iterative-improve close-enough? f)
  (define (try guess)
    (let ((next (f guess)))
      (if (close-enough? guess next)
          next
          (try next))))
  try)

(define (sqrt x)
  ((iterative-improve (lambda (guess next) (< (abs (- (square guess) x)) 0.001))
                      (lambda (guess) (average guess (/ x guess))))
   1.0))

(define tolerance 0.00001)

(define (fixed-point f guess)
  ((iterative-improve (lambda (v1 v2) (< (abs (- v1 v2)) tolerance)) f) guess))

(define (cube-root x)
  (fixed-point (average-damp (lambda (y) (/ x (square y)))) 1.0))
