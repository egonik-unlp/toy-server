import requests
import pandas as pd
import numpy as np
import time
url =  "http://localhost:3000"
N = 40
all_panics =[]
all_gaps = [0.0001, 0.001, 0.01,0.02,0.03,0.04,0.05,0.06,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1,1.2,1.3,1.4,1.5,2]
faileds = []
rights = []
def request(url: str, gap : float):
	failed = 0
	right = 0 
	for i in range(N):
		try:
			r = requests.get(url)
			right += 1
			if r.status_code != 200:
				print(r.content, r.status_code)
		except Exception as e:
			failed += 1
			all_panics.append(e)
		time.sleep(gap)
	return failed, right

for gap in all_gaps:
	failed, right = request(url, gap)
	faileds.append(failed)
	rights.append(right)

pd.DataFrame({"gap_seconds": all_gaps, "failed_attemts": faileds, "correctly_answered_requests" : rights}).to_csv("data.csv")

