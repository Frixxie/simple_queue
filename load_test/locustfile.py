from locust import HttpUser, task

class LoadTest(HttpUser):
    @task
    def enqueue(self):
        self.client.post("queue", data=b"1")

    @task
    def deqeue(self):
        self.client.get("queue")

    @task
    def length(self):
        self.client.get("length")
