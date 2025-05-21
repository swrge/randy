updateSecret:
	@if [ ! -f .env ]; then \
		echo "[INFO] .env file is not present"; \
	else \
		while IFS='=' read -r key value; do \
			if [ -n "$$key" ] && [ -n "$$value" ]; then \
				echo "[INFO] Updating secret: $$key"; \
				gcloud secrets create $$key --data-file=- --replication-policy=automatic <<< "$$value" || \
				gcloud secrets versions add $$key --data-file=- <<< "$$value"; \
			fi \
		done < .env; \
		echo "[INFO] pushed .env content to google cloud secret manager"; \
	fi
