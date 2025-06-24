function syncProblems() {
  fetch("/api/sync-problems", {
    method: "POST",
  }).then((response) => {
    console.log(response);
  });
}

function initListeners() {
  document
    .querySelector("button#sync-problems")
    .addEventListener("click", syncProblems);
}

initListeners();
