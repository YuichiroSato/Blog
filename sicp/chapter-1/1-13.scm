(define phi (/ (+ 1 (sqrt 5)) 2))

(define psi (/ (- 1 (sqrt 5)) 2))

(define (pow a n)
  (define (iter p i)
    (if (>= i n)
        p
        (iter (* a p) (+ i 1))))
  (iter 1 0))

;; (fib 1) = 1
;; (fib 2) = 1
;; (fib n) = (+ (fib (- n 1)) (fib (- n 2)))
;;         = (+ (/ (- (pow phi (- n 1)) (pow psi (- n 1))) (sqrt 5))
;;              (/ (- (pow phi (- n 2)) (pow psi (- n 2))) (sqrt 5)))
;;         = (/ (- (pow phi n) (pow psi n)) (sqrt 5))
(define (fib n)
  (truncate (/ (- (pow phi n) (pow psi n)) (sqrt 5))))

(define (print-fib n)
  (define (iter i)
    (format #t "~d " (fib i))
    (if (>= i n)
        '()
        (iter (+ i 1))))
  (iter 1))
