(define (main args)
  (display (car (cdr (car (cdr (cdr '(1 3 (5 7) 9)))))))
  (display (car (car '((7)))))
  (display (car (cdr (car (cdr (car (cdr (car (cdr (car (cdr (car (cdr '(1 (2 (3 (4 (5 (6 7)))))))))))))))))))
  (display "\n"))

