
import { backend } from "../src/declarations/backend";

window.pickWinner = async function () {
  const input = document.getElementById("names").value;
  const names = input.split("\n").map(n => n.trim()).filter(Boolean);

  console.log("Names:", names);
  if (names.length === 0) {
    document.getElementById("winner").innerText = "Please enter some names.";
    return;
  }

  const result = await backend.pick_winner(names);
  console.log("Winner:", result);
  document.getElementById("winner").innerText = `🏆 Winner: ${result}`;
};
