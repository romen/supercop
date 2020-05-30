#include <immintrin.h>
#include "SABER_params.h"
#include <string.h>
#include <stdint.h>
#include <stdio.h>
#include "SABER_indcpa.h"
#include "crypto_kem.h"
#include "kem.h"
#include "verify.h"
#include "randombytes.h"
#include "fips202.h"
#include "fips202x4.h"
#include "kem4x.c"

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

	// Redundant memory allocation to satisfy SaberX4 needs
  	uint8_t pk1[CRYPTO_PUBLICKEYBYTES], sk1[CRYPTO_SECRETKEYBYTES];
  	uint8_t pk2[CRYPTO_PUBLICKEYBYTES], sk2[CRYPTO_SECRETKEYBYTES];
  	uint8_t pk3[CRYPTO_PUBLICKEYBYTES], sk3[CRYPTO_SECRETKEYBYTES];

	uint8_t c1[CRYPTO_CIPHERTEXTBYTES], c2[CRYPTO_CIPHERTEXTBYTES], c3[CRYPTO_CIPHERTEXTBYTES];	
	uint8_t k1[CRYPTO_BYTES], k2[CRYPTO_BYTES], k3[CRYPTO_BYTES];


int crypto_kem_keypair(unsigned char *pk, unsigned char *sk)
{
	crypto_kem_keypair4x(pk, sk, pk1,sk1, pk2,sk2, pk3,sk3);
}


int crypto_kem_enc(unsigned char *c, unsigned char *k, const unsigned char *pk)
{
	crypto_kem_enc4x(c, k, pk, c1, k1, pk1, c2, k2, pk2, c3, k3, pk3);
}


int crypto_kem_dec(unsigned char *k, const unsigned char *c, const unsigned char *sk)
{
	crypto_kem_dec4x(k, c, sk, k1, c1, sk1, k2, c2, sk2, k3, c3, sk3);
}


