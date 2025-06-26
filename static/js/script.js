function syncProblems() {
  fetch("/api/sync-problems", {
    method: "POST",
  }).then((response) => {
    console.log(response);
  });
}

function login() {
  const username = document.querySelector("input#username")?.value;

  console.log(username);
  if (username?.length > 0) {
    document.cookie = "username=" + username;
  }
}

function initListeners() {
  const syncProblemsButton = document.querySelector("button#sync-problems");
  const loginButton = document.querySelector("button#signIn");

  if (syncProblemsButton) {
    syncProblemsButton.addEventListener("click", syncProblems);
  }

  console.log(loginButton);
  if (loginButton) {
    loginButton.addEventListener("click", login);
  }
}

initListeners();
