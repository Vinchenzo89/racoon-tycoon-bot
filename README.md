# Racoon Tycoon Bot
Bot that plays Racoon Tycoon (for those without friends to play Racoon Tycoon). Written in Rust

## To Do
Add ability to execute each possible turn action for any given player:
- [x] 1 Produce Comodities
- [x] 2 Sell Comodities
- [ ] 3 Buy Building
- [ ] 4 Buy Railroad
- [ ] 5 Auction a Railroad

Implement the actual Bot AI
- [ ] Assess the current game state to identity victory points earned from each possible action the Bot can take on its turn
  - [ ] Ensure Bot has as much visibility of the game state/board as any other player 
- [ ] Choose the turn action based on highest victory points gained
