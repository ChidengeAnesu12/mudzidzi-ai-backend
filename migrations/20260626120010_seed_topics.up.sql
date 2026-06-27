-- Seeds the exact same knowledge map used by the Flutter app's mock
-- data (MockKnowledgeMapRepository) so frontend and backend agree on
-- topic identity from day one. Slugs match KnowledgeNodeModel.id
-- values exactly.
INSERT INTO topics (slug, title, category, display_order) VALUES
    ('numbers',           'Numbers',           'numbers',      1),
    ('fractions',         'Fractions',         'numbers',      2),
    ('algebra',           'Algebra',           'algebra',      3),
    ('linear_equations',  'Linear Equations',  'algebra',      4),
    ('geometry',          'Geometry',          'geometry',     5),
    ('functions',         'Functions',         'functions',    6),
    ('trigonometry',      'Trigonometry',      'trigonometry', 7),
    ('quadratics',        'Quadratics',        'functions',    8),
    ('statistics',        'Statistics',        'statistics',   9);

-- Prerequisite edges, matching MockKnowledgeMapRepository exactly.
INSERT INTO topic_dependencies (topic_id, prerequisite_topic_id)
SELECT t.id, p.id FROM topics t, topics p
WHERE (t.slug, p.slug) IN (
    ('fractions', 'numbers'),
    ('algebra', 'fractions'),
    ('linear_equations', 'algebra'),
    ('geometry', 'numbers'),
    ('functions', 'linear_equations'),
    ('trigonometry', 'geometry'),
    ('quadratics', 'functions'),
    ('statistics', 'numbers')
);