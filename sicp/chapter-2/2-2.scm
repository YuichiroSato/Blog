(define (make-point x y)
  (cons x y))

(define (x-point p)
  (car p))

(define (y-point p)
  (cdr p))

(define (make-segment start end)
  (cons start end))

(define (start-segment segment)
  (car segment))

(define (end-segment segment)
  (cdr segment))

(define (avg a b)
  (/ (+ a b) 2))

(define (midpoint-segment segment)
  (let* ((start (start-segment segment))
         (end (end-segment segment))
         (x1 (x-point start))
         (y1 (y-point start))
         (x2 (x-point end))
         (y2 (y-point end)))
    (make-point (avg x1 x2) (avg y1 y2))))

(define (print-point p)
  (format #t "(~d,~d)" (x-point p) (y-point p)))
