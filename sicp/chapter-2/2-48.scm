(define (make-segment start start-to-end)
  (cons start start-to-end))

(define (start-segment segment)
  (car segment))

(define (end-segment segment)
  (cadr segment))
