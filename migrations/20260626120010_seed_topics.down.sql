DELETE FROM topic_dependencies
WHERE topic_id IN (SELECT id FROM topics WHERE slug IN (
    'numbers', 'fractions', 'algebra', 'linear_equations', 'geometry',
    'functions', 'trigonometry', 'quadratics', 'statistics'
));

DELETE FROM topics WHERE slug IN (
    'numbers', 'fractions', 'algebra', 'linear_equations', 'geometry',
    'functions', 'trigonometry', 'quadratics', 'statistics'
);