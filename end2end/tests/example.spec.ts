import { test, expect } from "@playwright/test";

test("homepage has the portfolio title", async ({ page }) => {
  await page.goto("http://localhost:8080/");

  await expect(page).toHaveTitle("CrawKatt · Minecraft modder & Rust developer");
});