# Bardado
Bot do Discord em Rust para tacar dados de rpg e tocar música de fundo

### Funcionalidades:

#### - Dados variáveis:
Ao mandar uma mensagem no chat como 2d8 o bot automáticamente rodará tal dado e responderá a mensagem original.
A formatação do dado é a seguinte: # Número de vezes (opcional) | Número de dados (Se vazio se torna 1) | d | Número de lados | Modificador (opcional)

#### - Player de música
Contém todas as funções padrão do meu outro bot, Sun Ca
- play: Toca uma música através de um nome ou link
- pause: Pausa a música
- resume: Continua a música
- skip: Pula a música atual
- stop: Para a música e desconecta do canal de voz
- queue: Mostra a fila de músicas
- clear: Limpa a fila de músicas
- swap: Troca a posição de duas músicas na fila
- remove: Remove uma música da fila
- join: Entra no canal de voz
- leave: Sai do canal de voz
- loop: Ativa ou desativa o loop da fila