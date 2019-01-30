(define (compose f g)
  (lambda (x) (f (g x))))

(define (repeated f n)
  (define (iter i result)
    (if (>= i n)
        result
        (iter (+ i 1) (compose f result))))
  (iter 1 f))

(define tolerance 0.00001)

(define (average x y) (/ (+ x y) 2.0))

(define (average-damp f)
  (lambda (x) (average x (f x))))

(define (fixed-point f first-guess)
  (define (close-enough? v1 v2)
    (< (abs (- v1 v2)) tolerance))
  (define (try guess)
    (let ((next (f guess)))
      (if (close-enough? guess next)
          next
          (try next))))
  (try first-guess))

(define (square x) (* x x))

(define (cube x) (* x x x))

(define (pow x n)
  (define (iter i result)
    (if (>= i n)
        result
        (iter (+ i 1) (* result x))))
  (iter 0 1))

(define (sqrt x)
  (fixed-point (average-damp (lambda (y) (/ x y))) 1.0))

(define (cube-root x)
  (fixed-point (average-damp (lambda (y) (/ x (square y)))) 1.0))

(define (fourth-root x)
  (fixed-point (average-damp (average-damp (lambda (y) (/ x (cube y))))) 1.0))

(define (nth-root x n)
  (fixed-point ((repeated average-damp n) (lambda (y) (/ x (pow y (- n 1))))) 1.0))

(define (optimal-nth-root x n)
  (fixed-point ((repeated average-damp (log n 2)) (lambda (y) (/ x (pow y (- n 1))))) 1.0))
