(define (filter-accumulate filter combiner null-value term a next b)
  (define (iter a1 result)
    (if (> a1 b)
        result
        (iter (next a1) (if (filter a1)
                            (combiner (term a1) result)
                            result))))
  (iter a null-value))

(define (true x) #t)

(define (sum term a next b) (filter-accumulate true + 0 term a next b))

(define (product term a next b) (filter-accumulate true * 1 term a next b))

(define (id x) x)

(define (inc x) (+ x 1))

(define (square x) (* x x))

;;(define (sum-of-squared-primes a b)
;;  (filter-accumulate prime? + 0 square a inc b))
;;
;;(define (sum-of-gcds n)
;;  (filter-accumulate (lambda (x) (= 1 (gcd x n))) * 1 id 1 inc n))
