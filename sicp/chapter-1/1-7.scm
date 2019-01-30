(define (sqrt-iter guess x)
  (display guess)
  (display "\n")
  (if (good-enough? guess x)
      guess
      (sqrt-iter (improve guess x)
                 x)))

(define (improve guess x)
  (average guess (/ x guess)))

(define (average x y)
  (/ (+ x y) 2))

(define (good-enough? guess x)
  (< (abs (- (square guess) x)) 0.001))

(define (sqrt x)
  (sqrt-iter 1.0 x))

(define (better-sqrt-iter guess x)
  (let ((guess1 (improve guess x)))
       (if (better-good-enough? guess guess1)
           guess
           (better-sqrt-iter guess1 x))))

(define (better-good-enough? guess0 guess1)
  (< (abs (- guess0 guess1)) (* guess0 0.001)))

(define (better-sqrt x)
  (better-sqrt-iter 1.0 x))
