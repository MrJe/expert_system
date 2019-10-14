## V A R I A B L E S
################################################################################
NAME			=	expert_system

## R U L E S
################################################################################
.PHONY			:	all clean fclean re

all				:	$(NAME)

$(NAME)			:
	cargo build --release
	cp target/release/expert_system .

clean			:
	cargo clean

fclean			:	clean
	rm -f $(NAME)

re				:	fclean
	make
