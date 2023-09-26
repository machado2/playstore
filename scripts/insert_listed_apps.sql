insert into listed_app
 (id, last_updated, title, score, min_installs, genre_id, free)
select id,
	now(),
	appdata->>'title',
	(appdata->>'score')::numeric,
	(appdata->>'minInstalls')::numeric,
	appdata->>'genreId' as genreId,
	appdata->>'free' = 'true'
from app
where 	appdata->>'offersIAP' = 'false'
	and appdata->>'adSupported' = 'false'
	and appdata->>'genreId' like 'GAME%'
	and (appdata->>'score')::numeric >= 3
