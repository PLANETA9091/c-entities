// Fake player for baseline runs: joins the offline-mode Paper server and
// stays AFK at spawn so Paper's Entity Activation Range sees a player.
// Protocol: the bot speaks 26.1 (the newest minecraft-data protocol with
// shipped data); ViaVersion + ViaBackwards translate 26.1 <-> 26.3
// server-side. ViaVersion ALONE refuses old clients ("only supports newer
// client versions") — ViaBackwards is REQUIRED, and its absence is why the
// bench bot was kicked "Outdated client" in EVERY 26.3 run (including
// doc 20's), leaving the A/B legs playerless with entities never
// activated. Both plugins are staged for BOTH A/B legs (paired fairness).
// Verified locally 2026-10-23: ViaVersion+ViaBackwards 5.12.2-SNAPSHOT on
// Paper 26.3 — BOT-LOGGED-IN + BOT-SPAWNED.
const mineflayer = require('mineflayer')

const host = process.env.BOT_HOST || '127.0.0.1'
const port = parseInt(process.env.BOT_PORT || '25565', 10)

function connect(attempt) {
  const bot = mineflayer.createBot({
    host,
    port,
    username: 'baseline_bot',
    auth: 'offline',
    version: process.env.BOT_VERSION || '26.1',
  })
  bot.on('login', () => console.log('BOT-LOGGED-IN'))
  bot.on('spawn', () => console.log('BOT-SPAWNED'))
  bot.on('kicked', (r) => console.log('BOT-KICKED', String(r).slice(0, 200)))
  bot.on('error', (e) => console.log('BOT-ERROR', e && e.message))
  bot.on('end', (r) => {
    console.log('BOT-END', String(r).slice(0, 200))
    if (attempt < 30) setTimeout(() => connect(attempt + 1), 3000)
  })
}

connect(0)
